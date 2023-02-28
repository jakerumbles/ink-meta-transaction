// import { expect } from '@jest/globals';
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from '@polkadot/keyring/types';
import InkMetaConstructor from "../typechain-generated/constructors/inkmetatransaction";
import InkMetaContract from "../typechain-generated/contracts/inkmetatransaction";
import FlipperConstructor from "../typechain-generated/constructors/flipper";
import FlipperContract from "../typechain-generated/contracts/flipper";
import { Transaction } from "../typechain-generated/types-arguments/inkmetatransaction";
import Web3 from "web3";
import { Result } from '@727-ventures/typechain-types';
import type { WeightV2 } from '@polkadot/types/interfaces';


describe('Ink Meta Transaction', () => {
    let api: ApiPromise;
    let alice: KeyringPair;
    let bob: KeyringPair;

    let inkMetaConstructor: InkMetaConstructor;
    let flipperConstructor: FlipperConstructor;

    let inkMetaContract: InkMetaContract;
    let flipperContract: FlipperContract;

    let INK_META_ADDRESS: any;
    let FLIPPER_ADDRESS: any;

    let FLIPPER_DEFAULT = false;

    let gasRequired: WeightV2;

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
        ({ api, alice: alice, bob: bob } = globalThis.setup);

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

    it('execute works', async () => {
        // Transaction to call the flip() fn in the Flipper contract
        let transaction: Transaction = {
            callee: FLIPPER_ADDRESS,
            selector: ["63", "3a", "a5", "51"],
            input: [],
            transferredValue: 100,
            gasLimit: 1000000000,
            allowReentry: false,
            nonce: 0,
            expirationTimeSeconds: Date.now() + 100000
        }

        let hashed_transaction = Web3.utils.soliditySha3(JSON.stringify(transaction));
        console.log(`Hashed transaction: ${hashed_transaction}`);

        let signature_buffer: number[] = [];
        // Get the signature into the right type as expected by `execute`
        let signature = alice.sign(hashed_transaction).forEach(b => {
            signature_buffer.push(b);
        });
        console.log(`Signature bytes: ${signature_buffer}`);

        revertedWith(
            await inkMetaContract.query.execute(transaction, signature_buffer),
            'BadSignature',
        );
        // ({ gasRequired } = await flipperContract
        //     .withSigner(alice)
        //     .query.flip());
        // console.log(`Gas for flip(): ${gasRequired}`);

        // await flipperContract
        //     .withSigner(alice)
        //     .tx.flip({ gasLimit: gasRequired });
        // expect((await flipperContract.query.get()).value.ok).toBe(!FLIPPER_DEFAULT);
    });
});

function revertedWith(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    result: { value: { err?: any } },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any,@typescript-eslint/explicit-module-boundary-types
    errorTitle: any,
): void {
    if (result.value instanceof Result) {
        result.value = result.value.ok;
    }
    if (typeof errorTitle === 'object') {
        expect(result.value).toHaveProperty('err', errorTitle);
    } else {
        console.log(`Err: ${result.value.err}`);
        expect(result.value.err).toHaveProperty(errorTitle);
    }
}