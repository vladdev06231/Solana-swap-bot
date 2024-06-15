import * as anchor from "@project-serum/anchor";

import {
  PublicKey,
  Keypair,
  Connection,
  Transaction,
  clusterApiUrl,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  TransactionSignature,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction

} from "@solana/web3.js";

import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  // @ts-ignore
  getAssociatedTokenAddress,
  // @ts-ignore
  createAssociatedTokenAccountInstruction,
  // @ts-ignore
  mintTo,
  // @ts-ignore
  createMint,
} from "@solana/spl-token";

// describe("solswap", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.Solswap as Program<Solswap>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });

import * as Constants from "./constants";
import { IDL } from "../target/types/solswap";
import * as keys from "./keys";


const connection = new Connection(clusterApiUrl(Constants.NETWORK));
let secretKey = Uint8Array.from([12,12,8,4,121,169,38,71,246,172,103,213,199,122,181,91,17,76,180,17,238,27,79,151,89,68,92,146,122,35,80,39,51,252,173,227,112,172,251,165,233,216,18,234,14,56,252,144,42,34,218,6,147,114,137,99,68,248,88,91,84,104,86,134]);



export const getProgram = () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.getProvider();
  // console.log("provider:", provider);
  //   connection,
  //   wallet,
  //   anchor.Provider.defaultOptions()
  // );
  const program = new anchor.Program(IDL, Constants.PROGRAM_ID, provider);
  // console.log("program:", program);
  return program;

};

const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
  'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
);

export const findAssociatedTokenAddress =  async (
    walletAddress: PublicKey,
    tokenMintAddress: PublicKey
): Promise<PublicKey> => {
    return (await PublicKey.findProgramAddress(
        [
            walletAddress.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            tokenMintAddress.toBuffer(),
        ],
        SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
    ))[0];
}

export const initializeProgram = async (
  wallet: Keypair
): Promise<string> => {
  if (wallet.publicKey === null) throw new Error();
  const program = getProgram();
  let poolKey = await keys.getPoolKey();
  const tx = new Transaction().add(await program.methods
    .initialize(
    //   wallet.publicKey,
    //   Constants.TREASURY,
    //   Constants.DEFAULT_TIER_DAYS,
    //   Constants.DEFAULT_TIER_PERCENT,
    //   Constants.DEFAULT_MAX_TIER
    )
    .accounts({
      admin: wallet.publicKey,
      settings: await keys.getSettingsKey(),
      botrole: await keys.getBotRoleKey(),
      pool: await keys.getPoolKey(),
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .instruction());
  const txHash = await sendAndConfirmTransaction(anchor.getProvider().connection, tx, [wallet]);
  console.log("Trasaction ", txHash);
  
  let poolBal = (await anchor.getProvider().connection.getBalance(poolKey)).toString();
  console.log("poolBal ", poolBal);

  return txHash;
};

let wrappedSolAccount: PublicKey | null = null;

export const swapToken = async (
  wallet: Keypair
): Promise<any> => {
  if (wallet.publicKey === null) throw new Error();
  const program = getProgram(); 
  let poolKey = await keys.getPoolKey();
  const tx = new Transaction().add(await program.methods.chargeSolFee(new anchor.BN(1000), new anchor.BN(10))
    .accounts({
      authority: wallet.publicKey,
      botrole: await keys.getBotRoleKey(),
      pool: poolKey,
      systemProgram: SystemProgram.programId
    })
    .instruction());
  
  const txHash = await sendAndConfirmTransaction(anchor.getProvider().connection, tx, [wallet]);

  console.log("Trasaction ", txHash);

  let poolBal = (await anchor.getProvider().connection.getBalance(poolKey)).toString();
  console.log("poolBal ", poolBal);

  return txHash;
};

export const withdrawPlatformFee = async (
  wallet: Keypair
): Promise<any> => {
  if (wallet.publicKey === null) throw new Error();
  const program = getProgram(); 
  let poolKey = await keys.getPoolKey();
  const tx = new Transaction().add(await program.methods.withdrawPlatformFee()
    .accounts({
      admin: wallet.publicKey,
      settings: await keys.getSettingsKey(),
      pool: await keys.getPoolKey(),
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY
    })
    .instruction());
  
  const txHash = await sendAndConfirmTransaction(anchor.getProvider().connection, tx, [wallet]);

  console.log("Trasaction ", txHash);

  
  let poolBal = (await anchor.getProvider().connection.getBalance(poolKey)).toString();
  console.log("poolBal ", poolBal);

  return txHash;

};

describe("solswap", () => {
  it("Is initialized!", async () => {
     let wallet = Keypair.fromSecretKey(secretKey);
    
     let airdropHash = await anchor.getProvider().connection.requestAirdrop(
      wallet.publicKey,
      1000_000_000_000
    );
    await anchor.getProvider().connection.confirmTransaction(airdropHash);


     const res = await initializeProgram(wallet);
     console.log("hxhash:", res);
  });

  it("token swap!", async () => {
    let wallet = Keypair.fromSecretKey(secretKey);
    let airdropHash = await anchor.getProvider().connection.requestAirdrop(
      wallet.publicKey,
      1000_000_000_000
    );
    await anchor.getProvider().connection.confirmTransaction(airdropHash);

    const res = await swapToken(wallet);
    console.log("hxhash:", res);

  });

  it("withdra platform fee", async () => {
    let wallet = Keypair.fromSecretKey(secretKey);
    let airdropHash = await anchor.getProvider().connection.requestAirdrop(
      wallet.publicKey,
      1000_000_000_000
    );
    await anchor.getProvider().connection.confirmTransaction(airdropHash);

    const res = await withdrawPlatformFee(wallet);
    console.log("hxhash:", res);
  });
});
