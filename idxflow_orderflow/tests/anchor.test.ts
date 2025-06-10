describe("idxflow orderflow tests", () => {
  const user = pg.wallet.publicKey;
  let globalStatePda: web3.PublicKey;
  let userAccountPda: web3.PublicKey;

  // SPL Token program ID (hardcoded to avoid import issues)
  const TOKEN_PROGRAM_ID = new web3.PublicKey(
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
  );

  // Replace these with actual public keys from your Playground UI
  const userTokenAccount = new web3.PublicKey("PUT_USER_TOKEN_ACCOUNT_HERE");
  const rewardVault = new web3.PublicKey("PUT_REWARD_VAULT_HERE");
  const stakingVault = new web3.PublicKey("PUT_STAKING_VAULT_HERE");

  before(async () => {
    [globalStatePda] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("global_state")],
      pg.program.programId
    );

    [userAccountPda] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("user_account"), user.toBuffer()],
      pg.program.programId
    );
  });

  it("Initializes global state", async () => {
    await pg.program.methods
      .initialize(new BN(100), new BN(60), new BN(5000)) // reward_rate, epoch_duration, min_volume
      .accounts({
        globalState: globalStatePda,
        authority: user,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
  });

  it("Creates user account", async () => {
    await pg.program.methods
      .createUserAccount()
      .accounts({
        userAccount: userAccountPda,
        user,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
  });

  it("Records swap volume", async () => {
    await pg.program.methods
      .recordSwapVolume(new BN(6000))
      .accounts({
        globalState: globalStatePda,
        userAccount: userAccountPda,
        user,
      })
      .rpc();
  });

  it("Stakes tokens", async () => {
    await pg.program.methods
      .stakeTokens(new BN(2_000_000_000)) // 2000 tokens (6 decimals)
      .accounts({
        globalState: globalStatePda,
        userAccount: userAccountPda,
        userTokenAccount,
        stakingVault,
        user,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  });

  it("Unstakes tokens", async () => {
    await pg.program.methods
      .unstakeTokens(new BN(1_000_000_000)) // 1000 tokens
      .accounts({
        globalState: globalStatePda,
        userAccount: userAccountPda,
        userTokenAccount,
        stakingVault,
        user,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  });

  it("Claims rewards", async () => {
    await pg.program.methods
      .claimRewards()
      .accounts({
        globalState: globalStatePda,
        userAccount: userAccountPda,
        userTokenAccount,
        rewardVault,
        user,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  });
});
