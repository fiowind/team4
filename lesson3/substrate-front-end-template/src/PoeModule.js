import React, { useEffect, useState } from 'react';
import { Form, Input, Grid } from 'semantic-ui-react';

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

  return (
    <Grid.Column width={8}>
      <h1>Poe Module</h1>
      <Form>
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
              inputParams: [digest,price],
              paramFields: [true]
            }}
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
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
        <div style={{ overflowWrap: 'break-word' }}>{`Claim info, owner: ${owner}, block_num: ${blockNum}, price: ${price} `}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
