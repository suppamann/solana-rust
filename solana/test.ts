// Mainnet: solana config set -um
// Devnet: solana config set -ud
// Testnet: solana config set -ut
// Localhost: solana config set -ul

export const achu = await fetch('https://api.devnet.solana.com',{
    method:"POST",
    headers: {"Content-Type": "application/json"},
    body:JSON.stringify( {
        jsonrpc: "2.0",
        method: "getAccountInfo",
        params:[
            'GF1j3TPspZFDNZrFoNnAZzF2DaRTqdmkNoHe5ZH3NMLX',
            {encoding: "jsonParsed"}
        ],
        id:"1"   
    })
})
.then(res => res.json())
.then(res => console.log("res"));