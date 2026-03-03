

export const achu = await fetch('https://api.devnet.solana.com', {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
        jsonrpc: "2.0",
        method: "getAccountInfo",
        params: [
            'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
            { encoding: "jsonParsed" }
        ],
        id: "1"
    })
})
    .then(res => res.json())
    .then(res => console.log(res));
    