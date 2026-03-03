import{ clusterApiUrl, Connection, Keypair } from "@solana/web3.js";

const connection = new Connection(clusterApiUrl("devnet"),"confirmed");

const keyPair = Keypair.fromSecretKey(Uint8Array.from([204,120,84,177,165,79,90,154,210,83,244,27,171,43,22,188,121,76,181,149,78,255,248,72,147,176,171,229,67,126,55,223,97,212,96,146,54,77,94,199,56,212,136,86,15,59,208,133,190,62,10,225,15,182,238,146,51,73,4,221,253,73,239,136]));

console.log(keyPair.publicKey.toBase58());

