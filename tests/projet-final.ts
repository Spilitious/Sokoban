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

const mapData = Buffer.from( [1, 1, 1, 1, 1, 1,
  1, 0, 0, 0, 0, 1,
  1, 0, 0, 0, 0, 1,
  1, 0, 0, 0, 0, 1,
  1, 0, 0, 0, 0, 1,
  1, 1 ,1 ,1, 1, 1]); 
const width = 6;
const height = 6;

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

  });
  
  it("Creation d'une map", async () => {
    
    await LoadFixtureForBuild();
    console.log("Ajout d'une caisse en 2-2");
    console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    console.log("Ajout de la position de départ du joueur en 4-3")

    let tx = await program.rpc.addItem(3,2,2,  {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    tx = await program.rpc.addItem(4,3, 3,  {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
 
   
    tx = await program.rpc.addItem(2,1, 1,  {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });

    let updatedGame = await program.account.gameState.fetch(gamePDA);
    let  updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData);


    console.log("Mouvement vers le bas");
    // console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    // console.log("Ajout de la position de départ du joueur en 4-3")

    tx = await program.rpc.movet(3, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    updatedGame = await program.account.gameState.fetch(gamePDA);
    updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData); 

    console.log("Mouvement vers la gauche");
    // console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    // console.log("Ajout de la position de départ du joueur en 4-3")

    tx = await program.rpc.movet(4, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    updatedGame = await program.account.gameState.fetch(gamePDA);
    updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData); 


    console.log("Mouvement vers le droite");
    // console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    // console.log("Ajout de la position de départ du joueur en 4-3")

    tx = await program.rpc.movet(2, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    updatedGame = await program.account.gameState.fetch(gamePDA);
    updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData); 

    console.log("Mouvement vers le haut");
    // console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    // console.log("Ajout de la position de départ du joueur en 4-3")

    tx = await program.rpc.movet(1, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    updatedGame = await program.account.gameState.fetch(gamePDA);
    updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData); 


    console.log("Mouvement vers le droite");
    // console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    // console.log("Ajout de la position de départ du joueur en 4-3")

    tx = await program.rpc.movet(2, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    updatedGame = await program.account.gameState.fetch(gamePDA);
    updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData); 

    console.log("Mouvement vers le bas");
    // console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    // console.log("Ajout de la position de départ du joueur en 4-3")

    tx = await program.rpc.movet(3, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    updatedGame = await program.account.gameState.fetch(gamePDA);
    updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData); 

    console.log("Mouvement vers le bas");
    // console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    // console.log("Ajout de la position de départ du joueur en 4-3")

    tx = await program.rpc.movet(3, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    updatedGame = await program.account.gameState.fetch(gamePDA);
    updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData); 

    console.log("Mouvement vers le gauche");
    // console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    // console.log("Ajout de la position de départ du joueur en 4-3")

    tx = await program.rpc.movet(4, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    updatedGame = await program.account.gameState.fetch(gamePDA);
    updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData); 

    console.log("Mouvement vers la droite");
    // console.log("Ajout de la position d'arrivé de la caisse en  en 3-3");
    // console.log("Ajout de la position de départ du joueur en 4-3")

    tx = await program.rpc.movet(2, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });
   
    updatedGame = await program.account.gameState.fetch(gamePDA);
    updatedMapData = updatedGame.mapData;
   
    displayMapData(updatedMapData); 

  });

  

});
