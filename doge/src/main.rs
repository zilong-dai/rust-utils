use sha2::{Digest, Sha256};

type Hash = [u8; 32];

fn sha256d(data: &Vec<u8>) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(result);
    hasher.finalize().into()
}

fn compute_merkle_root_from_path(index_hash: Hash, path_hashs: &Vec<Hash>, index: isize) -> Hash {
    if index == -1 {
        return [0u8; 32];
    }

    let mut result = index_hash;
    let mut index = index;

    for path_hash in path_hashs.iter() {
        let data = match index & 1 {
            0 => {
                let mut data = result.to_vec();
                data.extend_from_slice(path_hash);
                data
            }
            1 => {
                let mut data = path_hash.to_vec();
                data.extend_from_slice(&result);
                data
            }
            _ => panic!("invalid index"),
        };
        result = sha256d(&data);
        index >>= 1;
    }
    result
}

fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::*;

    #[test]
    fn test_sha256d() {
        let data = hex::decode("f6e498f123a66cc16f864cc930894615595750b1a5d77a9d7623a1bc0e1fbf6b")
            .expect("decode failed");
        assert_eq!(
            hex::encode(sha256d(&data)),
            "c76f9bf3d5d706edb5dd7b91a491419de98d1f97cc1cf806e84553a0deaf9e3c"
        );
    }

    #[test]
    fn test_compute_merkle_path() {
        // 5214403.json
        {
            let mut index_hash: [u8; 32] =
                hex::decode("0ee5478728c534ec88183cc6e6ef3c7283c01984ac2cd43076eb4cb16d844a22")
                    .expect("decode failed")
                    .try_into()
                    .expect("try_into Hash failed");
            index_hash.reverse();
            let path_hashs = [
                "0000000000000000000000000000000000000000000000000000000000000000",
                "f98c4e9736d8eb8bb46299798906695c755369a3df99a93ffdded1713f1cf6e2",
                "48e55233b9707330def98c80a2105eaa5fac8f687d872ffb4b4741fa2bdb247d",
                "c77d206e2f3cc38e18b85aaa89a8f142a9bf0f4106925d39708f91083e7d8594",
            ]
            .iter()
            .map(|hex_str| {
                let mut path_hash = hex::decode(hex_str).expect("decode failed");
                path_hash.reverse();
                path_hash.try_into().expect("try_into Hash failed")
            })
            .collect::<Vec<Hash>>();

            assert_eq!(
                hex::encode(compute_merkle_root_from_path(index_hash, &path_hashs, 8)),
                "fe607b64019bc62f2e7bc5683df965b0fc1512c5a8743bb25556e6b03305f6bb"
            );
        }

        {
            let mut index_hash: [u8; 32] =
                hex::decode("96e599b232634127f37a2b7411410bdf6da550eeffb4f575eef7fd6574ccc143")
                    .expect("decode failed")
                    .try_into()
                    .expect("try_into Hash failed");
            index_hash.reverse();
            let path_hashs = [
                "609be52156c593eb67526c53f0baf4b227e45854ed12e4c080a57ecbe9bdf327",
                "d8e093ca35667c8fced733859935161a0cce387c8074a59357d011672116fec2",
                "37ad2d7dab020d8eb659cf6ba4cd1d568ed40b38d46ed6715ab099d258b67f16",
                "62dd52377569cfb10b6740dfffb58dd55a6a43e6b804d5423a255452e6e82e82",
                "688a68122c3f881ef32ae8b6321dbcccb7f7577ecd79155aa6fa31c406b6abe9",
                "2ac907e2537b6bb6be42183edbef7d5aec75b53fba689ae437483f72671deaef",
                "11e9425f492abde9f16b31c3e6dc1cb420e137cb0e966643c99fc4f491fe281e",
                "4649e7ddb5d9c9ead36e5c03d3d6b59b313ed7e0e425a23d225b778a3b5c7bb6",
                "57ba2aa1e09db59e54c3f38794a50f2b0eefebd9bd01e1704649fa596049ca5a",
                "4c6662a24d9b42649bcc9026586a2fb30014493e5761246479c69fb301e5b2ee",
                "80f2bdc98149edbc22bb8b87bf7c7df3077473e4e4d9a0a8fcb24fc18e73a4e4",
            ]
            .iter()
            .map(|hex_str| {
                let mut path_hash = hex::decode(hex_str).expect("decode failed");
                path_hash.reverse();
                path_hash.try_into().expect("try_into Hash failed")
            })
            .collect::<Vec<Hash>>();

            assert_eq!(
                hex::encode(compute_merkle_root_from_path(index_hash, &path_hashs, 0)),
                "1621972afec0198088b73f6a123fba3d77a316ee48b27cab85f6e340e8bbcc05"
            );
        }
    }
}
