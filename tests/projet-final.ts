import { ProjetFinal } from './../target/types/projet_final';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { Keypair, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";


const program = anchor.workspace.ProjetFinal as Program<ProjetFinal>;
const connection = new Connection('http://localhost:8899', 'confirmed');
anchor.setProvider(anchor.AnchorProvider.env());

let authority =Keypair.generate();
const wallet = Keypair.generate();
 // Configure the client to use the local cluster.
let bump:number =255;

const mapData = Buffer.from( [1, 1, 1, 1, 1,
  1, 0, 0, 0, 1,
  1, 0, 0, 0, 1,
  1, 0, 0, 0, 1,
  1, 1 ,1 ,1, 1]); 
const width = 5;
const height = 5;

 let gamePDA = (Keypair.generate()).publicKey;
 

function getNextBump() {
  bump -=1;
  return bump;
}

function displayMapData(mapData:ArrayBuffer) {
  console.log("Map data:");
  for (let i = 0; i < height; i++) {
    let line = "";
    for (let j = 0; j < width; j++) {
      line += mapData[i * width + j] + " ";
    }
    console.log(line.trim());
  }

}

async function LoadFixtureForInit() {

  const lamports = 10 * LAMPORTS_PER_SOL;
  const tx = await connection.requestAirdrop(wallet.publicKey, lamports);
  await connection.confirmTransaction(tx);

  const tx3 = await connection.requestAirdrop(authority.publicKey, lamports);
  await connection.confirmTransaction(tx3);
  const balance2 = await connection.getBalance(wallet.publicKey);
}

async function LoadFixtureForBuild() {

  LoadFixtureForInit();
  console.log(getNextBump());
  const lamports = 10 * LAMPORTS_PER_SOL;
  authority =Keypair.generate();
  const tx3 = await connection.requestAirdrop(authority.publicKey, lamports);
  await connection.confirmTransaction(tx3);
  const balance2 = await connection.getBalance(wallet.publicKey);
  const seeds = [authority.publicKey.toBuffer()];

    [gamePDA, bump] = await PublicKey.findProgramAddress(
    seeds,
    program.programId,
    // Use a specific bump value, such as a timestamp or a counter
  
  );

  console.log("Bump", bump)
    
    const tx2 = await program.rpc.initialize(width, height, mapData, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });

}



describe("projet-final", () => {
 
  // beforeEach(async function() {
    // await LoadFixtureForInit();

 //});

  it("Is initialized!", async () => {
    await LoadFixtureForInit();
    // Find the PDA for the game account
    const [gamePDA, bump] = await PublicKey.findProgramAddress(
    [authority.publicKey.toBuffer()],
    program.programId);

    const tx2 = await program.rpc.initialize(width, height, mapData, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });


    let updatedGame = await program.account.gameState.fetch(gamePDA);
    let updatedMapData = updatedGame.mapData;
    displayMapData(updatedMapData);

    console.log("Ajout d'une caisse en 1-2")
    const tx = await program.rpc.addItem(2,1,2,  {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });

     updatedGame = await program.account.gameState.fetch(gamePDA);
     updatedMapData = updatedGame.mapData;
    // Log mapData with line breaks at each width
    displayMapData(updatedMapData);


  });
  
  it("Ajout d'un élément!", async () => {
    
    await LoadFixtureForBuild();
    console.log("Ajout d'une caisse en 1-2")
    const tx = await program.rpc.addItem(2,1,2,  {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });

    const updatedGame = await program.account.gameState.fetch(gamePDA);
    const updatedMapData = updatedGame.mapData;
    // Log mapData with line breaks at each width
    displayMapData(updatedMapData);


  });

});
