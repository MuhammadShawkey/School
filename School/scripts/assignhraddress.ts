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

const program = new anchor.Program(idl, provider);

  // Derive the PDAs

let [schoolPda] =  anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("school")],
  program.programId
);
const ceo = provider.wallet;

const main = async () => {
  const hrname = "Hassona" ;
  const hraddress = new anchor.web3.PublicKey("9dotJgijqADsTaRRZ5mQUSZpRx5VidS8ch6E9DEuL96y");
  
 
     const tx = await program.methods
      .assignhraddress(hrname , hraddress)
      .accounts({
        ceo: ceo.publicKey,
        school: schoolPda,
       systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    };

   main().catch(console.error);
