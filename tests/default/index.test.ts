import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import DefaultFactory from "./typedContract/constructors/default";
import Default from "./typedContract/contracts/default";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

use(chaiAsPromised);

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("default test", () => {
  let defaultFactory: DefaultFactory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  
  let contract: Default;
  const initialState = true;

  before(async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri("//Alice");

    defaultFactory = new DefaultFactory(api, deployer);

    contract = new Default(
      (await defaultFactory.new(initialState)).address,
      deployer,
      api
    );
  });

  after(async function tearDown() {
    await api.disconnect();
  });
});
