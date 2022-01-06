const assert = require("assert");
const anchor = require('@project-serum/anchor');
const { AssetTransfersCategory } = require("@alch/alchemy-web3");
const { SystemProgram } = anchor.web3;

describe('mysolanaapp', () => {
  // create and set a Provicder
  const provider = anchor.Provider.env();
  let _baseAccount;
  anchor.setProvider(provider);
  const program = anchor.workspace.Mysolanaapp;
  it("Creates a counter", async () => {
    // call 'create' function via RPC
    const baseAccount = anchor.web3.Keypair.generate();
    await program.rpc.create({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    // fetch account and check value of count
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Count 0:', account.count.toString())
    assert.ok(account.count.toString() == 0);
    _baseAccount = baseAccount;
  });

  it("Increments the counter", async () => {
    const baseAccount = _baseAccount;

    await program.rpc.increment({
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Count 1: ', account.count.toString())
    assert.ok(account.count.toString() == 1);
  });
});