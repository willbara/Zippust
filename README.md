# Zippust - A rust based file encryption program.

Zippust is a file encryption tool designed to be lightweight while packed with features. The program is aiming to be visually simple and efficient, allowing users to easily drag and drop their files into the application, choose an encryption method, and secure their sensitive data. Inspired by [Picocrypt](https://github.com/Picocrypt/Picocrypt).

![Preview](./assets/preview_1-0.gif)

---

## Roadmap

- **File Selection & Basic AES-256 Encryption**: Implement file selection (drag-and-drop or file picker) and basic AES-256 encryption with password-based key derivation.
  
- **XChaCha20 & RSA Encryption**: Integrate XChaCha20 cipher with Argon2id for key derivation, and add RSA encryption with public-private key pairs.

- **Decryption for AES, XChaCha20 & RSA**: Enable decryption for all implemented encryption methods.

- **Advanced Features**: Add password protection for encryption keys, algorithm selection in settings. Create optional settings for compressing, file distrubution, and more.

- **UI & Platform Enhancements**: Improve UI because I'm not a UI/UX Dev.


## Contributing

Contributions are welcome! Feel free to submit pull requests or open issues to improve functionality or security.


## License

This project is licensed under the MIT License.

---

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [eframe](https://docs.rs/eframe/latest/eframe/)
- [egui](https://docs.rs/egui/latest/egui/)
- [image crate](https://docs.rs/image/latest/image/)
- [rand crate](https://docs.rs/rand/latest/rand/)
- [Argon2](https://docs.rs/argon2/latest/argon2/)
- [RSA Algorithm](https://en.wikipedia.org/wiki/RSA_(cryptosystem))

---
