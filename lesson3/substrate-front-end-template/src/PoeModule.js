import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Tab } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';


function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNum, setblockNum] = useState(0);
  const [price, setPrice] = useState(0);
  const [receiver, setReceiver] = useState('');

  // The currently stored value
  // const [currentValue, setCurrentValue] = useState(0);
  // const [formValue, setFormValue] = useState(0);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      // The storage value is an Option<u32>
      // So we have to check whether it is None first
      // There is also unwrapOr
      // if (newValue.isNone) {
      //   setCurrentValue('<None>');
      // } else {
      //   setCurrentValue(newValue.unwrap().toNumber());
      // }
      setOwner(result[0].toString());
      setblockNum(result[1].toNumber());
      setPrice(result[2].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    }).catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  const handleFileChosen = (file) => {
    const fileReader = new FileReader();
    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result)).map((b) => b.toString(16).padStart(2, '0')).join('');
      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    };
    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  };


  const panes = [
    {
      menuItem: '创建存证',
      render: () => <Tab.Pane attached={false}><Form>
        <Form.Field>
          <Input
            label='Your File'
            id='file'
            type='file'
            onChange={ (e) => handleFileChosen(e.target.files[0]) }
          />
        </Form.Field>

        <Form.Field>
          <Input
            label='Price'
            id='price'
            type='text'
            value={price}
            onChange={ (e) => setPrice(e.target.value) }
          />
        </Form.Field>

        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Create Claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, price],
              paramFields: [true]
            }}
          />
        </Form.Field>
        </Form>
      </Tab.Pane>
    },
    {
      menuItem: '删除存证',
      render: () => <Tab.Pane attached={false}><Form>
        <Form.Field>
          <Input
            label='claim'
            id='claim'
            type='text'
            value={digest}
            onChange={ (e) =>  setDigest(e.target.value) }
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Remove Claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'removeClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
        </Form.Field>
        </Form>
      </Tab.Pane>,
    },
    {
      menuItem: '转让存证',
      render: () => <Tab.Pane attached={false}>
        <Form>
        <Form.Field>
          <Input
            label='claim'
            id='claim'
            type='text'
            value={digest}
            onChange={ (e) =>  setDigest(e.target.value) }
          />
        </Form.Field>
        <Form.Field>
          <Input
            label='receiver'
            id='receiver'
            type='text'
            value={receiver}
            onChange={ (e) =>  setReceiver(e.target.value) }
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest,receiver],
              paramFields: [true]
            }}
          />
        </Form.Field>
        </Form>
      </Tab.Pane>,
    },
    {
      menuItem: '修改价格',
      render: () => <Tab.Pane attached={false}>
        <Form>
        <Form.Field>
          <Input
            label='claim'
            id='claim'
            type='text'
            value={digest}
            onChange={ (e) =>  setDigest(e.target.value) }
          />
        </Form.Field>
        <Form.Field>
          <Input
            label='Price'
            id='price'
            type='text'
            value={price}
            onChange={ (e) => setPrice(e.target.value) }
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Update Claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'updateClaimPrice',
              inputParams: [digest,price],
              paramFields: [true]
            }}
          />
        </Form.Field>
        </Form>
      </Tab.Pane>,
    },
    {
      menuItem: '购买存证',
      render: () => <Tab.Pane attached={false}>
        <Form>
        <Form.Field>
          <Input
            label='claim'
            id='claim'
            type='text'
            value={digest}
            onChange={ (e) =>  setDigest(e.target.value) }
          />
        </Form.Field>
        <Form.Field>
          <Input
            label='Price'
            id='price'
            type='text'
            value={price}
            onChange={ (e) => setPrice(e.target.value) }
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Buy Claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'buyClaim',
              inputParams: [digest,price],
              paramFields: [true]
            }}
          />
        </Form.Field>
        </Form>
      </Tab.Pane>,
    },
  ]


  return (
    <Grid.Column width={8}>
      <h1>存证模块</h1>
      <Tab menu={{ secondary: true, pointing: true }} panes={panes} />
        <table class="ui very padded table">
          <tbody>
            <tr>
              <td>status</td>
              <td style="word-break: break-all;">{status}</td>
            </tr>
            <tr>
              <td>claim</td>
              <td style="word-break: break-all;">{digest}</td>
            </tr>
            <tr>
              <td>owner</td>
              <td>{owner}</td>
            </tr>
            <tr>
              <td>blockNum</td>
              <td>{blockNum}</td>
            </tr>
            <tr>
              <td>price</td>
              <td>{price} Uint</td>
            </tr>
           
          </tbody>
        </table>


    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
