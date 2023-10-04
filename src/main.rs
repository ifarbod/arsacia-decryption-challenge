// Copyright (c) 2023 iFarbod. All Rights Reserved.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::convert::TryInto;
use z85::{decode, DecodeError};

fn decrypt_data(encoded_data: &[u8]) -> Vec<u8>
{
    let mut decrypted_data = vec![0; encoded_data.len() / 8];
    let source = encoded_data
        .chunks_exact(8)
        .map(|chunk| u64::from_ne_bytes(chunk.try_into().unwrap()));
    let mut previous_byte = 1;

    for (i, chunk) in source.enumerate()
    {
        let key = ((chunk >> 32) as u32 & u32::MAX) as u32;
        let mut decrypted_byte = (chunk as u32) ^ key;
        decrypted_byte ^= i as u32;
        decrypted_byte /= 2 * previous_byte;

        decrypted_data[i] = decrypted_byte as u8;
        previous_byte = decrypted_data[i] as u32;
    }

    decrypted_data
}

fn main()
{
    // TODO(iFarbod): Take from stdin, content from another command piped in
    let encoded_data = r#"ag6PwXWf{g=u02Fdnr&8*y%6>0yDo<0{cot/%EQp/gQ+YnG0fC5nr7KrDo3DugCQY*V=z8}$bSGSjvU*Gdz>5+qa##4Y4vk2F59yX=>YUc$b-2g7bRK}4b6}2Ab16ooLY@8>3](y@DLttr/ar6^oNc&KxeQe$-Gj^-kdJ7)mWZUgn@(2znC@MvO0=y%I6hc$se+=Ph$l[7p!A4$N)+LPj>v.3b1R8u<S$J2ucL**I#nUqXllwy.!r>7B=(}4&Df1AA]vjTeo6R--uR1+E>@?KU<?4xKF%vEXn+qZCwLuuYtTyFFWc8YAIOKE:WCNAS%9[ra*XJklL6nDIhVQ/oTY[l}ZKL6AwY-*4L2c5-&2kq)>}T7uT+KnU72DB5Q[.Ol5ah1pK3fq0(*/WktyOSAL!^B[qo+}OnH!edlpqG[F!CZL>qPCmV[oaq8/oYsr6Y0P^?]p(0QY8zy1WzNbiadEx$J?w!L9S5mptFF*m59..<lUBWhNLr1N?lQdNF8ad%t#x(b8CYMqKXQtrf?^2eNIka$tnuOm[1:PV{VVsNu<%}Sa.!?#QQnhHj^Ts%96i/#^hzV.S.5e&J1?k>!LgP1U2G@SN6ijy<G$Su7AuBcUUFgJBqWHp7d}0+9laR(F0DUEwzQVadV!0IAA8g1B{mbc*^D[IB{-#SHkPth-GBI2hUd>uA{:(0q^K4kJYH:HI=:j!4msx6VxgdmB+(aC!gR4{!ozi^Gz9E1kcJbv>TvM%c<.$09/Py1l/44yNb0hN6VLT+0CXmCoN0bFWvK:(YtI7y8zWZJ1.Tk[&6gDJ>3.H0(o4kNDMUF}YBBM^T}ZNYlbT+n?Jw>JRASC)>4&AfyAMkx5{N2w@ZR$Gy+gODy]Z9J*RMH9S1ZvqAb^0-?uf04UuWqKn46Yp6yv7so%GgaA{!An:fp?W:e>LSEBWYLvbphq>%&>fCt=R$DOb!9t5K)FjretJKv<H5/?l2LnfhErspH{#iUXlZG}/%kRN[d[<mFD.*OZe.&cw3M1.7gZ!//1P[hLyfm)WG>a}$r)8R}V#>-r0GXNpV#lnaP1cDPHF!r9#>5#[nbD9GWAx&Ol}1Xq.Q!Qx./ar0EH-WZ8NxFFRLnTwaKcxu&3{#=5]nuVCgEOw?[/b-@yW5OIRuH&GDQWk<Ap$STi[FN1C1U2a<5MxYl00j97D8%SBw6dAJsnxDq2cLME9ep!mv:hk]&QE)YUaM+dW9@cCt+1]+x<.:K=o-twA%e@nVMV^vM(la.vmHEQwMbABN*bYe8BESXwM@o<J%tP[pAGIBtafXOHSK^V@FiM6Kkh[=}!RQ)CY/vD=a&L038^6zi?d%@siAv&Rlj]x1ikH=HZZJ}>uCg#vm?el)/s55Blm*#INE>XN:KE^5xEc4Tk?hpaPxv$ky&Ke]wonxB5FBRT{pxBqgnM>I68<Av@{r@en%imJ$6G[uL.4Cw+(cZ{ojoL^7&n^@f*l<cA4tv>G:M8v+exZ^1f41gX1k!Sj#9O[tLkFaO<[#5IxpN-Y6a/0Z@OO*fygz/5dq]M:4qRFBA7lVA<6aIpiLnC8NIiJfi{qL/--5:j*uh^1.vrJ9bvHl<z{X]2oOzwTzxi6BBb3tU1cQ(z?(B3:W>]2={VMS<bhJ!{Iblh6MXphh/[uF.wFe-mq{qa<w1orrDRygAfZo=?hyUVy(rrM^aPfHNWvyW&dE1{eBlBl?A%ySQioIodch5PGeFn&Hk>+QW<J6yRjuJU?rZW3V0*j14ZGZuo]AP1.5r}U+qB+9q*g8>^=l58k8Jst.#2*U9qG]@jv*?qpps8cTnHMeE15R)bvBjV9iS}YBu>63ktG5576GsXxThoAw]x9qbX$5zXk=J779&yg?zzlJ^MNNZU)zN)5[TIN4<n4i{=J/(j7v*DSvXv92fp[w{$N1<)v>&X}C)9Jn/#c+.Is*ZBaxiEMz?V!GYX=s2>)LVKV^fsw!>)0d0!(-an8}d{-J?U0@.8ZKh8Ob}>r>r>$5?vu)gD4q4Gk-Ex+X[&pS)}Y{9:bm{k<Cx60#]3wbZ1=s59[%q/RZkLI(Yp1)XC7(eTqSWNQt..]Q0)9X?nT8F4@pB8xifmk[[H$7zwkAybzw)0+}#ZBeUFT!7jLj7$wJ8*.h!+F=BP=@t-9mFp))h-z]+Sy#lD>=?)#%8hRHyz9oKP/YmoZk89-AJK*h?EEF{RYJ^E)$+I2E/0^t][rNt&*Sw@LFmh&loVQVxKuV^D)rHGdB1.uTe^h2r29!#TW8msla$<yGuN>vjSq1:Qp]2usWvtMW)yYi4+cz9HZ{!#o=ZCH4y4gjOk>1s8<lr>P%K8q($khB^B4Z.lx5Dt6j4to*4>Tg$y4l8HxHtAs/fU-$fko+3u(iAtsGH)n$Os<F8Zz0o2ACc/s]VhQZvP5=<?X=t4.3k=7}U[U@m97O+9=Zcjv=b84&15@}}[j^NT1=][?[SHcTW*-MDn[Owk{xi[o32Se.=8z:vQrK-o4%82yd?M5NCuxWj3i=l%SgR.{Z%H-^t+kjTF6RyS^0M7W*!m%mI/a/9{f:ehZf}Jj0c]7nz/.@CO&%avDS&XS?eLZ51ivB:4+^(83X5(uWoBu{uHPU5/@M(YBV%@/NkYodECG.]+?O3?-u4?Um&UtdA3-Q6W(ht9OHSzirxn?KS19ruo=NUC+$U]hdp6UDxrT$.Jg&?#CxkEt8!Q>aIo*jaJ[OJqrE7H)=UorcgFDMW@9Ojn"#;

    match decode(encoded_data)
    {
        Ok(decoded_data) =>
        {
            println!("Decoded Z85 data: {:?}", decoded_data);

            let dest = decrypt_data(&decoded_data);
            let st = String::from_utf8_lossy(&dest);

            println!("Decrypted string: {:?}", st);
        }
        Err(err) => match err
        {
            DecodeError::InvalidLength(length) =>
            {
                println!("Z85 data length ({}) is not a multiple of five", length)
            }
            DecodeError::InvalidByte(position, byte) =>
            {
                println!(
                    "Z85 data has an invalid byte (0x{:02X}) at position {}",
                    byte, position
                )
            }
            DecodeError::InvalidChunk(position) =>
            {
                println!(
                    "Z85 data has an invalid 5-byte chunk at position {}",
                    position
                )
            }
            DecodeError::InvalidTail =>
            {
                println!("Z85 data has an invalid padding chunk")
            }
            _ =>
            {
                println!("An unknown error occurred during Z85 decoding")
            }
        },
    }
}
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn verify_decryption()
    {
        let encoded_data = "ag6PwXWf{g"; // 'S' => 1f b3 d8 1f b9 b3 d8 1f

        match decode(encoded_data)
        {
            Ok(decoded_data) =>
            {
                let decrypted_bytes = vec![31, 179, 216, 31, 185, 179, 216, 31];
                assert_eq!(decoded_data, decrypted_bytes);

                let decrypted_data = decrypt_data(&decoded_data);
                let decrypted_string = String::from_utf8_lossy(&decrypted_data);

                assert_eq!(decrypted_string, "S");
            }
            _ =>
            {}
        }
    }
}
