# Hashing: SHA256,MD5

Symmetric encryption: AES  
Asymmetric encryption: RSA, EdDSA(SOL), ECDSA(ETH and BTC)
     is what is used in web3, it generated public key and private keys
     ECDSA-Elliptic curve cryptography -> ED25519
     EdDSA-Edwards curve digital signature algorithm -> secp256k1

# Hierarchical Deterministic (HD) Wallets:
     Type of wallet that can generate a tree of key pairs from a single seed.
     this way multiple addresses (or public/private key pairs) can be created from a single seed, provides convenience.

# Mnemonics:
     Human readable string of words from which cryptographic seeds are generated.

# Proof of Work (PoW):
     Bitcoin is a PoW blockchain.

     The network timestamps transaction by hashing them into the ongoing chain of hash based proof of work, forming a record that cannot be changed without redoing the proof of work.

     The longest chain not only servers as proof of sequence of events witnessed, but proof that it came from the largest pool of CPU power. As long as a majority of CPU power is controlled by nodes that are not cooperating to attack the network, they will generate the longest chain and outspace the attackers, this is the the concensus on which the Bitcoin exists.

# Seed phrase:
     The 12/24 words/mnemonics from which a seed would be generated
Seed:
     The encoded string that is generated using the seed phrase
Derivation Path:
     All coin type uses a specific number to identify themselves, 0 being bitcoin, 60 eth, 501 solana
     m/44'/501'/${i}'/0'

     ' (called Prime) is used for Hardened Derivation, (read as 501 prime)
     no ' -> normal derivation, meaning you can generate a child public key using only the parent public key
     hardened derivation is where you need both the parent pub and pvt keys to generate child

     m/purpose/coin_type/account/change/address_index
     m --> seed/root
     44 --> BIP 44
     ${i} --> account number , like 1, 2 etc

     change --> only relevant to Bitcoin not for eth/sol
          suppose user1 has 1 BTC, transfers 0.1 BTC to user2, then 0.9 BTC becomes UTXO (unspent transaction output) which would have its own address
     
     address_index --> you have two accounts a and b. sub accounts under a -> a1, a2 are address_index

# Accounts in Solana

Program Account: Executable,The actual smart contract (e.g., the Jupiter Aggregator or a custom Rust program).

System Account: Non-Executable,A standard Wallet. It holds SOL and is owned by the System Program.

Data Account: Non-Executable,A generic account created by a program to store its state (e.g., a Counter account).

Mint Account: Non-Executable,The factory for a token. It defines the name, decimals, and total supply (e.g., the USDC Mint).

Token Account: Non-Executable,An account that holds a specific user's balance for a specific Mint (e.g., your USDC balance). 

PDA (Program Derived Address): Non-Executable,A special account with no private key. It is controlled entirely by a Program.

# Mint

Mint Account : Global definition of a specific token(USDC). It does not store the list of users, it stores the rules of the token like decimal places (SOL has 9, USDC 6) , total number of tokens in existence, who is allowed to mint more tokens or freeze existing accounts.

Token Account: In solana the main wallet cannot hold SPL tokens, it holds only SOL, so when you have multiple tokens, for each a new Token account would be created, dedicated to a specific mint.

ATA (Associated Token Account): 
A regular token account can have any random address, which would be a nightmare for devs, so ATA was introduced. Its in the ATA's data field where the number for tokens you hold will be defined, when you make a transaction token balance would be updated in the senders and receivers ATA data fields. For each token a certain user can/will have only one ATA.

is technically botth a token account in terms of its data structure and a PDA in terms of its address as its dervied deterministically unlike the wallet which has a pvt-pub key pair. ATA_Address = f(User_Wallet_Address, Token_Mint_Address), since this formula is publically available ATA address can be calculated without asking the user or new account can be created if it doesnt exist.



# https://animotvslash.org/jujutsu-kaisen-the-culling-game-part-1-season-3-episode-5/

# things to read about --> 
     1> turbine protocol  
     2> gulf of stream protocol
     3> Stake-Weighted Quality of Service (QoS)