import * as anchor from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { AnchorProvider, Wallet, Idl } from '@coral-xyz/anchor';
import fs from 'fs';
import BN from "bn.js";
 
// Setup connection and wallet
const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"), "confirmed");

const walletKeypair = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync("/home/mohamed/righthraddress.json", "utf8")))
);
const wallet = new Wallet(walletKeypair);
const provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());
anchor.setProvider(provider);

// Load IDL and program
const idlString = fs.readFileSync("/home/mohamed/Projects/elmadrasa/target/idl/elmadrasa.json", "utf8");
const idl = JSON.parse(idlString) as Idl;
const program = new anchor.Program(idl, provider);

 
// ---------------------- Main Script ----------------------

const [schoolPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("school")],
  program.programId
);

const [studentsPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("students")],
  program.programId
);

const [treasurydash] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasurydash")],
      program.programId
    );

const main = async () => {
  // The logic inside here will be able to use `await`
  
  const student_name = "Shawkey";
  const class_index = 0;
  const receipient_account = new PublicKey("4DzmVqkqKQfZzsdzgEXoihX2SseGAtQiLtQrPuUMmekv");
  const score = new BN(100);
  const reason = "batekhaaa"
    
  const tx = await program.methods
    .evaluateStudent(student_name, class_index, score , reason)
    .accounts({
      signer: walletKeypair.publicKey,
      school: schoolPda,
      students: studentsPda,
      treasurydash: treasurydash,
      receipientAccount: receipient_account,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  }

  main().catch(console.error);


