// import { expect } from '@jest/globals';
import { ApiPromise } from "@polkadot/api";
import { Keyring } from '@polkadot/keyring';
import { KeyringPair } from '@polkadot/keyring/types';
import * as crypto from '@polkadot/util-crypto';
import InkMetaConstructor from "../typechain-generated/constructors/inkmetatransaction";
import InkMetaContract from "../typechain-generated/contracts/inkmetatransaction";
import FlipperConstructor from "../typechain-generated/constructors/flipper";
import FlipperContract from "../typechain-generated/contracts/flipper";
import { Transaction } from "../typechain-generated/types-arguments/inkmetatransaction";
import Web3 from "web3";
import * as $ from "scale-codec"
import { Result } from '@727-ventures/typechain-types';
import type { WeightV2 } from '@polkadot/types/interfaces';


describe('Ink Meta Transaction', () => {
    let api: ApiPromise;
    let keyring: Keyring;
    let alice: KeyringPair;
    let bob: KeyringPair;

    let inkMetaConstructor: InkMetaConstructor;
    let flipperConstructor: FlipperConstructor;

    let inkMetaContract: InkMetaContract;
    let flipperContract: FlipperContract;

    let INK_META_ADDRESS: string;
    let FLIPPER_ADDRESS: string;

    let FLIPPER_DEFAULT = false;

    let gasRequired: WeightV2;



    // let transaction: Transaction = {
    //     callee: FLIPPER_ADDRESS,
    //     selector: ["63", "3a", "a5", "51"],
    //     input: [],
    //     transferredValue: 100,
    //     gasLimit: 1000000000,
    //     allowReentry: false,
    //     nonce: 0,
    //     expirationTimeSeconds: Date.now() + 100000
    // }

    // let pairFactory: Pair_factory;
    // let factoryFactory: Factory_factory;
    // let routerFactory: Router_factory;
    // let tokenFactory: Token_factory;
    // let wnativeFactory: Wnative_factory;

    // let pairHash: Hash;
    // let factory: Factory;
    // let router: Router;
    // let [token0, token1]: Token[] = [];
    // let wnative: Wnative;


    async function setup(): Promise<void> {
        ({ api, keyring: keyring, alice: alice, bob: bob } = globalThis.setup);

        // Create instance of constructors, that will be used to deploy contracts
        // Constructors contains all constructors from the contract
        inkMetaConstructor = new InkMetaConstructor(api, alice);
        flipperConstructor = new FlipperConstructor(api, alice);

        // Deploy contract via constructor
        INK_META_ADDRESS = (await inkMetaConstructor.default()).address;
        FLIPPER_ADDRESS = (await flipperConstructor.new(FLIPPER_DEFAULT)).address;

        console.log('Ink Meta Transaction contract deployed at:', INK_META_ADDRESS);
        console.log('Flipper contract deployed at:', FLIPPER_ADDRESS);

        inkMetaContract = new InkMetaContract(INK_META_ADDRESS, alice, api);
        flipperContract = new FlipperContract(FLIPPER_ADDRESS, alice, api);
    }

    it('flip works', async () => {
        await setup();
        expect((await flipperContract.query.get()).value.ok).toBe(FLIPPER_DEFAULT);

        ({ gasRequired } = await flipperContract
            .withSigner(alice)
            .query.flip());
        console.log(`Gas for flip(): ${gasRequired}`);

        await flipperContract
            .withSigner(alice)
            .tx.flip({ gasLimit: gasRequired });
        expect((await flipperContract.query.get()).value.ok).toBe(!FLIPPER_DEFAULT);
    });

    it('verify works', async () => {
        // const $transaction_codec = $.object(
        //     $.field("callee", $.str),
        //     $.field("selector", $.array($.str)),
        //     $.field("input", $.array($.str)),
        //     $.field("transferredValue", $.u128),
        //     $.field("gasLimit", $.u64),
        //     $.field("allowReentry", $.bool),
        //     $.field("nonce", $.u128),
        //     $.field("expirationTimeSeconds", $.u64)
        // );


        let decoded_address = keyring.decodeAddress(FLIPPER_ADDRESS);
        console.log(decoded_address);

        let decoded_addr_arr: number[] = [];
        decoded_address.forEach(b => {
            decoded_addr_arr.push(b);
        });
        console.log(decoded_addr_arr);

        let selector: number[] = [99, 58, 165, 81];
        let input: number[] = [];
        let transferredValue: number = 100;
        let gasLimit: number = 1000000000;
        let allowReentry: boolean = false;
        let nonce: number = 0;
        let expirationTimeSeconds: number = 1677782453176 + 100000000;

        // Transaction to call the flip() fn in the Flipper contract
        let transaction: Transaction = {
            callee: decoded_addr_arr,
            selector: selector /* [0x63, 0x3a, 0xa5, 0x51]*/,
            input: input,
            transferredValue: transferredValue,
            gasLimit: gasLimit,
            allowReentry: allowReentry,
            nonce: nonce,
            expirationTimeSeconds: expirationTimeSeconds
        }

        const $transaction_codec = $.object(
            $.field("callee", $.sizedUint8Array(32)),
            $.field("selector", $.sizedUint8Array(4)),
            $.field("input", $.uint8Array),
            $.field("transferredValue", $.u128),
            $.field("gasLimit", $.u64),
            $.field("allowReentry", $.bool),
            $.field("nonce", $.u128),
            $.field("expirationTimeSeconds", $.u64)
        );

        let transaction_for_encoding = {
            callee: decoded_address,
            selector: Uint8Array.from(selector),
            input: Uint8Array.from(input),
            transferredValue: BigInt(transferredValue),
            gasLimit: BigInt(gasLimit),
            allowReentry: transaction.allowReentry,
            nonce: BigInt(nonce),
            expirationTimeSeconds: BigInt(expirationTimeSeconds)
        }

        let encoded_transaction = $transaction_codec.encode(transaction_for_encoding);
        console.log(`Encoded transaction Uint8Array: ${encoded_transaction}`);
        console.log(`Encoded transaction toString(): ${encoded_transaction.toString()}`);

        // let hashed_transaction = Web3.utils.soliditySha3(encoded_transaction.toString());
        let hashed_transaction = crypto.keccak256AsU8a(encoded_transaction);
        console.log(`Hashed transaction: ${hashed_transaction}`);

        let signature_buffer: number[] = [];
        // Get the signature into the right type as expected by `execute` or `verify`
        let signature = alice.sign(hashed_transaction).forEach(b => {
            signature_buffer.push(b);
        });
        console.log(`Signature bytes: ${signature_buffer}`);

        let res = await inkMetaContract.query.verfiy(transaction, signature_buffer);
        let res_value = res.value;
        console.log(res_value.ok);
    });




    // it('execute works', async () => {
    //     // Transaction to call the flip() fn in the Flipper contract
    //     let transaction: Transaction = {
    //         callee: FLIPPER_ADDRESS,
    //         selector: ["63", "3a", "a5", "51"],
    //         input: [],
    //         transferredValue: 100,
    //         gasLimit: 1000000000,
    //         allowReentry: false,
    //         nonce: 0,
    //         expirationTimeSeconds: Date.now() + 100000
    //     }

    //     let hashed_transaction = Web3.utils.soliditySha3(JSON.stringify(transaction));
    //     console.log(`Hashed transaction: ${hashed_transaction}`);

    //     let signature_buffer: number[] = [];
    //     // Get the signature into the right type as expected by `execute`
    //     let signature = alice.sign(hashed_transaction).forEach(b => {
    //         signature_buffer.push(b);
    //     });
    //     console.log(`Signature bytes: ${signature_buffer}`);

    //     revertedWith(
    //         await inkMetaContract.query.execute(transaction, signature_buffer),
    //         'BadSignature',
    //     );
    //     // ({ gasRequired } = await flipperContract
    //     //     .withSigner(alice)
    //     //     .query.flip());
    //     // console.log(`Gas for flip(): ${gasRequired}`);

    //     // await flipperContract
    //     //     .withSigner(alice)
    //     //     .tx.flip({ gasLimit: gasRequired });
    //     // expect((await flipperContract.query.get()).value.ok).toBe(!FLIPPER_DEFAULT);
    // });
});

function revertedWith(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    result: { value: { err?: any } },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any,@typescript-eslint/explicit-module-boundary-types
    errorTitle: any,
): void {
    if (result.value instanceof Result) {
        console.log("First if");
        result.value = result.value.ok;
    }
    if (typeof errorTitle === 'object') {
        expect(result.value).toHaveProperty('err', errorTitle);
    } else {
        console.log(`Err: ${result.value.err}`);
        expect(result.value.err).toHaveProperty(errorTitle);
    }
}