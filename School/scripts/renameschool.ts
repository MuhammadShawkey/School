import * as anchor from '@coral-xyz/anchor';
import { Keypair, Transaction } from '@solana/web3.js';
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

 // Derive the PDAs
 let [schoolPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("school")],
  program.programId
);
 

const main = async () => {
  const newschoolname = "madrasty gameela" ;
     const tx = await program.methods
      .renameschool(newschoolname)
      .accounts({
        ceo: ceo.publicKey,
        school: schoolPda,
      })
      .rpc();
    
 
    console.log("✅ Transaction signature:", tx);
    console.log(`School renamed correctly and its new name is '${schoolname}'`);

  };


  main().catch(console.error);