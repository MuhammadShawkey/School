import * as anchor from '@coral-xyz/anchor';
import { Keypair, Transaction } from '@solana/web3.js';
import { AnchorProvider, Wallet, web3, Idl } from '@coral-xyz/anchor';
import fs from 'fs';

// Set up connection to the Solana devnet
const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl('devnet'), 'confirmed');

const walletKeypair = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync("/home/mohamed/Projects/elmadrasa/id.json", "utf8")))
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
  let [schoolPda] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("school")],
    program.programId
  );
 

const main = async () => {

 
     const tx = await program.methods
      .terminatehraddress()
      .accounts({
        ceo: ceo.publicKey,
        school: schoolPda,
       systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
 
    console.log("✅ Transaction signature:", tx);
    console.log("HR is terminated. Cleared hrname and hraddress in the school account #Warning you need to assign new Hr."
    );
  };

  main().catch(console.error);
