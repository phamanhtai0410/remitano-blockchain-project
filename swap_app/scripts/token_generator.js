import { 
    clusterApiUrl,
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey
} from '@solana/web3.js';
import { 
    AccountLayout,
    TOKEN_PROGRAM_ID,
    createMint,
    getMint,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    getAccount
} from "@solana/spl-token";
import * as dotenv from 'dotenv';

async function main() {
    

    const payer = Keypair.generate();
    const mintAuthority = Keypair.generate();
    const freezeAuthority = Keypair.generate();
    console.log("* Payer : ", JSON.stringify(payer));
    const connection = new Connection(
        clusterApiUrl('testnet'),
        'confirmed'
    );

    /**
    *      Get some airdrop to `payer`
    */
    const airdropSignature = await connection.requestAirdrop(
        payer.publicKey,
        LAMPORTS_PER_SOL,
    );
    await connection.confirmTransaction(airdropSignature);
    console.log("* Airdrop :", airdropSignature);

    /**
    *      Create the fungible token
    */
    const mint = await createMint(
        connection,
        payer,
        mintAuthority.publicKey,
        freezeAuthority.publicKey,
        9
    );  
    console.log('* New fungible token : ', mint.toBase58());

    /**
    *      Log the mint infomations
    */
    const mintInfo = await getMint(
        connection,
        mint
    );
    console.log('* New token supply : ', mintInfo.supply);

    /**
    *      Create an account to hold the new token balance
    */
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey
    )
    console.log('* New token account : ', tokenAccount.address.toBase58());

    /**
    *      Check the infomations of the token account
    */
    const tokenAccountInfo = await getAccount(
        connection,
        tokenAccount.address
    );
    console.log('* New token account supply: ', tokenAccountInfo.amount);

    /**
    *      Mint some token to the token holding account
    */
    await mintTo(
        connection,
        payer,
        mint,
        tokenAccount.address,
        mintAuthority,
        100000000000 // decimals is 9
    )
    console.log("* Done mint for token account !");

    /**
    *      Re-check the `supply` and `balance`
    */
    const mintInfo2 = await getMint(
        connection,
        mint
    )
    console.log("* New token Supply : ", mintInfo2.supply);

    const tokenAccountInfo2 = await getAccount(
        connection,
        tokenAccount.address
    )
    console.log("* Token acccount amount : ", tokenAccountInfo2.amount);

    /**
    *      View all tokens that own
    */
    (async () => {

        const connection = new Connection(clusterApiUrl('testnet'), 'confirmed');
        dotenv.config();
        const tokenAccounts = await connection.getTokenAccountsByOwner(
            new PublicKey(
                process.env.MY_ADDRESS
            ),
            {
            programId: TOKEN_PROGRAM_ID,
            }
        );

        console.log("Token                                         Balance");
        console.log("------------------------------------------------------------");
        tokenAccounts.value.forEach((tokenAccount) => {
            const accountData = AccountLayout.decode(tokenAccount.account.data);
            console.log(`${new PublicKey(accountData.mint)}   ${accountData.amount}`);
        })

    })();
}

main()






