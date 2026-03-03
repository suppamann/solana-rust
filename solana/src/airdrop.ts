import { Connection, LAMPORTS_PER_SOL, clusterApiUrl, PublicKey } from "@solana/web3.js"

const connection = new Connection(clusterApiUrl('devnet'));

async function myAirdrop(publicKey: string, amount: number) {
    const airdropSignature = await connection.requestAirdrop(new PublicKey(publicKey), amount);
    await connection.confirmTransaction({signature:airdropSignature})
}

myAirdrop("7atNhkWdAgQSJRauihSnsP1oaSAmoQu8z3vDgQyXVADD", LAMPORTS_PER_SOL).then(sig => {
    console.log('Airdrop signature: ', sig);
})
