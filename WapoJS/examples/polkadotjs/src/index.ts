import { Keyring, ApiPromise, HttpProvider } from '@polkadot/api';
import { waitReady } from '@polkadot/wasm-crypto';
import { stringToU8a, u8aToHex } from '@polkadot/util';

async function main() {
    console.log('waiting the crypto to be ready');
    await waitReady();
    console.log('crypto ready');

    /// Keyring
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
    console.log(`${alice.meta.name}: has address ${alice.address} with publicKey [${alice.publicKey}]`);
    const message = stringToU8a('this is our message');
    const signature = alice.sign(message);
    const isValid = alice.verify(message, signature, alice.publicKey);
    console.log(`The signature ${u8aToHex(signature)}, is ${isValid ? '' : 'in'}valid`);

    /// RPC
    const url = 'https://poc6.phala.network/ws';
    const provider = new HttpProvider(url);
    console.log(`connecting to ${url}`);
    const api = await ApiPromise.create({ provider });
    console.log('genesis hash:', api.genesisHash.toHex());
    console.log('ed:', api.consts.balances.existentialDeposit.toJSON());
}

main().catch(console.error).finally(() => process.exit());
