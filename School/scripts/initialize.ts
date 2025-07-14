import * as anchor from '@coral-xyz/anchor';
import { Keypair, PublicKey, Transaction } from '@solana/web3.js';
import { AnchorProvider, Wallet, web3, Idl } from '@coral-xyz/anchor';
import fs from 'fs';

// Set up connection to the Solana devnet
const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl('devnet'), 'confirmed');

const walletKeypair = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync("/home/mohamed/id.json", "utf8")))
  );
//set the keypair as your wallet
const wallet = new Wallet(walletKeypair);  

// Set up the provider with the connection and a wallet
const provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());
anchor.setProvider(provider);

//read idl file ( similar to abi file in ethereum )
const idlString = fs.readFileSync("target/idl/elmadrasa.json", "utf8");

const idl = JSON.parse(idlString) as Idl;
const schoolname = "ta7ya masr"

const ceo = provider.wallet;
const program = new anchor.Program(idl, provider);

const main = async () => {
  // Derive the PDAs
    const [schoolPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("school")],
      program.programId
    );

    const [studentsPda] =  anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("students")],
      program.programId
    );

   
    const ix = await program.methods.initialize(schoolname)
    .accounts({ ceo: ceo.publicKey,
        school: schoolPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        students: studentsPda,
     })
    .rpc();
  
  };

  main().catch(console.error);
