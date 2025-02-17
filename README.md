# Solana Basic

[Build on Solana - Superteam Korea](https://lu.ma/4dfqedrp)

## Program examples

[solana-developers/program-examples](https://github.com/solana-developers/program-examples.git)

```bash
git clone https://github.com/solana-developers/program-examples.git
```

### Basics

> **account-data**

- `CreateAddressInfo` Accounts 구조체와 인스트럭션 함수

```rs
#[derive(Accounts)] // 어노테이션은 Anchor가 인스트럭션 실행 전 계정 검증 코드를 자동으로 생성하게 해준다.
pub struct CreateAddressInfo<'info> {
    #[account(mut)] // mut이니 변경 가능
    payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = ANCHOR_DESCRIMINATOR_SIZE + AddressInfo::INIT_SPACE,
    )]
    address_info: Account<'info, AddressInfo>,

    system_program: Program<'info, System>, // 시스템 프로그램 계정
}

pub fn create_address_info(
    ctx: Context<CreateAddressInfo>, // 인스트럭션 실행에 필요한 모든 계정 정보를 담고 있는 컨텍스트
    name: String,
    house_number: u8,
    street: String,
    city: String,
) -> Result<()> {
    *ctx.accounts.address_info = AddressInfo {
        name,
        house_number,
        street,
        city,
    };
    Ok(()) // 종료 시 호출
}

// instructions/create.rs
```

- `AccountInfo` 온체인 데이터 구조체

```rs
#[account] // 해당 구조체가 온체인에 저장되는 계정 데이터임을 명시
#[derive(InitSpace)] // Anchor가 컴파일 시 구조체의 각 필드 크기를 기반으로 필요한 메모리 공간을 자동 계산해주는 매크로
pub struct AddressInfo {
    #[max_len(50)] // 문자열 최대 길이 제한 (50자)
    pub name: String, // 저장: 4바이트 (길이) + 최대 50바이트 (내용)
    pub house_number: u8, // 1바이트
    #[max_len(50)]
    pub street: String, // 4바이트 + 최대 50바이트
    #[max_len(50)]
    pub city: String, // 4바이트 + 최대 50바이트
}

// state/address_info.rs
```

- 프로그램 모듈과 인스트럭션 연결

```rs
declare_id!("GpVcgWdgVErgLqsn8VYUch6EqDerMgNqoLSmGyKrd6MR"); // 고유 ID를 선언


// Anchor 프레임워크에서 이 모듈 내부에 정의된 함수들은 인스트럭션(프로그램의 엔트리 포인트)으로 간주
// 여기서는 create_address_info 인스트럭션을 정의하며, 이는 내부의 create 모듈에 있는 실제 로직 함수(create::create_address_info)를 호출
#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn create_address_info(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        create::create_address_info(ctx, name, house_number, street, city)
    }
}

// libs.rs
```
