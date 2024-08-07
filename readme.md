## setup - kurulum

`anchor init` çalıştırarak anchor-counter adında bir counter oluşturun

```sh
anchor init anchor-counter
```

yeni dizine geçin ve anchor-build çalıştırın

```sh
anchor build
```


Anchor build, yeni programınız için bir anahtar çiftini de oluşturur - anahtarlar target/deploy dizininde saklanır.

lib.rs dosyasını açın ve declare_id! makrosunu kontrol edin:

```ts
declare_id!("BouTUP7a3MZLtXqMAm1NrkJSKwAjmid8abqiNjUyBJSr");
```

run `anchor keys sync`

```sh
anchor keys sync
```

Anchor, her iki anahtarı da günceller:

lib.rs dosyasındaki declare_id!() makrosunda kullanılan anahtar.
Anchor.toml dosyasındaki anahtar.
Bu anahtarlar, anchor build sırasında oluşturulan anahtarla eşleşecek şekilde güncellenir.

```sh
Found incorrect program id declaration in "anchor-counter/programs/anchor-counter/src/lib.rs"
Updated to BouTUP7a3MZLtXqMAm1NrkJSKwAjmid8abqiNjUyBJSr
 
Found incorrect program id declaration in Anchor.toml for the program `anchor_counter`
Updated to BouTUP7a3MZLtXqMAm1NrkJSKwAjmid8abqiNjUyBJSr
 
All program id declarations are synced.
```

Finally, delete the default code in lib.rs until all that is left is the following:

```rust
use anchor_lang::prelude::*;
 
declare_id!("your-private-key");
 
#[program]
pub mod anchor_counter {
    use super::*;
 
}
```

### Implement - Uygulamak

İlk olarak, #[account] özniteliğini kullanarak yeni bir Counter hesap türü tanımlayalım. Counter yapısı, u64 türünde bir count alanı tanımlar. Bu, Counter türünde başlatılan herhangi bir yeni hesabın uyumlu bir veri yapısına sahip olmasını bekleyebileceğimiz anlamına gelir. #[account] özniteliği ayrıca, yeni bir hesap için ayrıcıyı (discriminator) otomatik olarak ayarlar ve hesabın sahibini declare_id! makrosundan alınan program ID'si olarak belirler.


```rust
#[account]
pub struct Counter {
    pub count: u64,
}
```

###  Bağlam türünü uygulayın - ımplement context type ınitialize

#[derive(Accounts)] makrosunu kullanarak ``Initialize`` türünü uygulayabiliriz. Bu tür, ``initialize`` talimatında kullanılan hesapları listeleyecek ve doğrulayacaktır. ``Initialize`` türü aşağıdaki hesapları içerecek:

1. ``counter`` - Talimatta başlatılan Counter hesabı.

2. ``user`` - Başlatma işlemi için ödeme yapan hesap.

3. ``system_program`` - Yeni hesapların başlatılması için gerekli olan sistem programı.

İşte bu hesapları listeleyen ve doğrulayan Initialize türünü tanımlayan bir örnek:


```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

Açıklamalar:

- ``#[derive(Accounts)]:`` Initialize yapısı, initialize_counter işlevi tarafından kullanılan hesapları tanımlar.

- ``pub counter: Account<'info, Counter>:`` Bu, Counter türünde bir hesap olup, talimat sırasında başlatılır. payer = user ifadesi, bu hesabın başlatılması için ödemeyi yapan kullanıcı hesabını belirtir.

- ``pub user: Signer<'info>:`` Hesap başlatma işlemi için ödeme yapan hesap, Signer özelliğiyle işaretlenir, böylece kullanıcı talimatı imzalayabilir ve işlemi onaylayabilir.

- ``pub system_program:`` Program<'info, System>: Bu, yeni hesapların başlatılması için gerekli olan Solana sistem programıdır.

Bu yapı, initialize_counter işlevinin doğru hesapları kontrol etmesini ve doğrulamasını sağlar, böylece başlatma işlemi sırasında doğru hesapların ve programların kullanıldığından emin olunabilir.

###  Add the `initialize` instruction - başlangıç talimatını ekleyin

Şimdi initialize talimatını #[program] içinde uygulayabiliriz. Bu talimat, Initialize türünden bir Context alır ve ek talimat verisi gerektirmez. Talimatın mantığında, Counter hesabının count alanını 0 olarak ayarlıyoruz.


```rust 
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.count = 0;
    msg!("Counter Account Created");
    msg!("Current Count: { }", counter.count);
    Ok(())
}
```


Açıklamalar:

- initialize işlevi:

``ctx: Context<Initialize>:`` Bu, Initialize yapısına dayalı hesap bağlamını sağlar.

- ``let counter = ``&mut ctx.accounts.counter;: counter hesap referansını alır ve count alanına erişim sağlar.

- ``counter.count = 0;:`` count alanını 0 olarak ayarlar. Bu, Counter hesabının başlangıç değerini ayarlar.

Bu talimat, Counter hesabını başlatırken count alanını 0 olarak ayarlayacak şekilde yapılandırılmıştır. Başlatma işlemi sırasında başka bir veri veya parametreye gerek kalmadan bu işlem gerçekleştirilir.

### Implement Context type Update #

#[derive(Accounts)] makrosunu kullanarak Update türünü oluşturalım. Bu tür, increment talimatı için gerekli olan hesapları listeleyecek ve doğrulayacaktır. Update türü aşağıdaki hesapları içerecektir:

1. ``counter`` - Artırılacak mevcut Counter hesabı.

2. `` user ``- İşlem ücreti için ödeme yapan hesap.

Bu hesapların gerekli kısıtlamalarını da #[account(..)] özniteliği ile belirteceğiz. İşte bu türün nasıl tanımlanacağına dair bir örnek:

```rust
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}
```

Açıklamalar:
- ``#[derive(Accounts)]:`` Update yapısı, increment işlevi tarafından kullanılan hesapları tanımlar.

    - ``pub counter:`` Account<'info, Counter>: Bu hesap mevcut bir Counter hesap olup, increment işlevi tarafından güncellenebilir (mut).

    - ``pub user:`` Signer<'info>: İşlem ücreti ödeyen hesap, Signer özelliği ile işaretlenir, böylece kullanıcı talimatı imzalayabilir ve işlemi onaylayabilir.

- ``#[account(mut)]: `` Bu öznitelik, hesapların güncellenebilir olduğunu belirtir. counter ve user hesapları increment işlevi sırasında güncellenebilir olmalıdır.

Bu yapı, increment işlevinin doğru hesapları kontrol etmesini ve doğrulamasını sağlar, böylece işlemin doğru hesaplar ve kısıtlamalarla gerçekleştirildiğinden emin olunur.

###  Add increment instruction #

Son olarak, #[program] içinde bir increment talimatı ekleyelim. Bu talimat, ilk talimat tarafından başlatılan bir Counter hesabının sayacını artırmak için kullanılacaktır. Bu talimat, Update türünden bir Context gerektirir (bir sonraki adımda uygulanacaktır) ve ek talimat verisi almaz. Talimatın mantığında, mevcut Counter hesabının count alanını 1 artırıyoruz.

```rust
pub fn increment(ctx: Context<Update>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    msg!("Previous counter: {}", counter.count);
    counter.count = counter.count.checked_add(1).unwrap();
    msg!("Counter incremented. Current count: {}", counter.count);
    Ok(())
}
```

### 7. Build 
All together, the complete program will look like this:

```rust
use anchor_lang::prelude::*;
 
declare_id!("BouTUP7a3MZLtXqMAm1NrkJSKwAjmid8abqiNjUyBJSr");
 
#[program]
pub mod anchor_counter {
    use super::*;
 
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter account created. Current count: {}", counter.count);
        Ok(())
    }
 
    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}
 
#[account]
pub struct Counter {
    pub count: u64,
}
```
### testing

Anchor testleri genellikle Typescript entegrasyon testleri olup mocha test çerçevesini kullanır. Test yapmayı daha ayrıntılı olarak daha sonra öğreneceğiz, ancak şimdilik anchor-counter.ts dosyasına gidip varsayılan test kodunu aşağıdaki kod ile değiştirin:

```rust
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { AnchorCounter } from "../target/types/anchor_counter";
 
describe("anchor-counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
 
  const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>;
 
  const counter = anchor.web3.Keypair.generate();
 
  it("Is initialized!", async () => {});
 
  it("Incremented the count", async () => {});
});
```


Yukarıdaki kod, başlatacağımız counter hesabı için yeni bir anahtar çiftini oluşturur ve her talimat için test yer tutucuları oluşturur.

Sonraki adımda, initialize talimatı için ilk testi oluşturun:

```rust
it("Is initialized!", async () => {
  // Add your test here.
  const tx = await program.methods
    .initialize()
    .accounts({ counter: counter.publicKey })
    .signers([counter])
    .rpc();
 
  const account = await program.account.counter.fetch(counter.publicKey);
  expect(account.count.toNumber()).to.equal(0);
});
```

Sonraki adımda, increment talimatı için ikinci testi oluşturun:

```rust
it("Incremented the count", async () => {
  const tx = await program.methods
    .increment()
    .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
    .rpc();
 
  const account = await program.account.counter.fetch(counter.publicKey);
  expect(account.count.toNumber()).to.equal(1);
});
```

Son olarak, anchor test komutunu çalıştırın ve aşağıdaki çıktıyı görmelisiniz:

```sh
anchor-counter
✔ Is initialized! (290ms)
✔ Incremented the count (403ms)
 
 
2 passing (696ms)
```

anchor test komutunu çalıştırmak, otomatik olarak yerel bir test doğrulayıcısı başlatır, programınızı dağıtır ve mocha testlerinizi bu programa karşı çalıştırır. Şu anda testlerle ilgili kafanız karışmışsa endişelenmeyin; daha sonra daha ayrıntılı olarak inceleyeceğiz.

Tebrikler, Anchor framework'ü kullanarak bir Solana programı oluşturmuş oldunuz! Daha fazla zaman ihtiyaç duyarsanız [çözüm koduna](https://github.com/Unboxed-Software/anchor-counter-program/tree/solution-increment) başvurabilirsiniz.

