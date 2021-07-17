# Enterprise-LocalContentServer-WebAdmin

## Backend
Backend is build with Rust, Bash, and Sqlite.

### Documentations

* GET API Documentations [GET API Documentations](api-doc/GET.md)

* POST API Documentations [POST API Documentations](api-doc/POST.md)

* PUT API Documentations [PUT API Documentations](api-doc/PUT.md)

* DELETE API Documentations [POST API Documentations](api-doc/DELETE.md)

### Installations

#### On Raspberry Pi

* Install Rust toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rust.sh
chmod +x rust.sh
./rust.sh
> 1
```

* Setup Backend

```bash
git clone https://github.com/koompi/enterprise-lcs-webadmin.git
cd enterprise-lcs-webadmin
cargo run
```

`Available at port 8080 of Raspberry PI`

***