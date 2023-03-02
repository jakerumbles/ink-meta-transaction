import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import InkMetaConstructor from "../typechain-generated/constructors/inkmetatransaction";
import InkMetaContract from "../typechain-generated/contracts/inkmetatransaction";
import FlipperConstructor from "../typechain-generated/constructors/flipper";
import FlipperContract from "../typechain-generated/contracts/flipper";
import { Transaction } from "../typechain-generated/types-arguments/inkmetatransaction";
import Web3 from "web3";
import { Result } from '@727-ventures/typechain-types';

// Create a new instance of contract
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'ecdsa' });
export default async function setupApi(): Promise<void> {
    const api = await ApiPromise.create({ provider: wsProvider });

    // const aliceKeyringPair = keyring.addFromAddress("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');



    globalThis.setup = { api, keyring, alice, bob };
}