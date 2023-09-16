describe("Test", () => {
  const newAccountKp = new web3.Keypair();
  it("initialize", async () => {
    const txHash = await pg.program.rpc.initialize({
      accounts: {
        counterAccount: newAccountKp.publicKey,
        user: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [newAccountKp],
    });
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction

  });

  it("increment", async () => {
    // Call the increment instruction
    const INT = "1"
    const txHash = await pg.program.rpc.increase(new anchor.BN(INT), {
      accounts: {
        counterAccount: newAccountKp.publicKey,
      },
    });
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);
  });

  it("decrement", async () => {
    // Call the increment instruction
    const INT = "1"
    const txHash = await pg.program.rpc.decrease(new anchor.BN(INT), {
      accounts: {
        counterAccount: newAccountKp.publicKey,
      },
    });
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);
  });
});
