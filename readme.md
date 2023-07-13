# BevyでRustGPUを使いたい(WIP)
現状エラーで動いてないです
spirv-optのremove-unused-interface-variables passが走っているかも
    https://github.com/EmbarkStudios/spirv-tools-rs/blob/main/src/opt/tool.rs
    -O, -Os, --legalize-hlsl には含まれていないはず
ShaderRefには対応していない

[bevy-rust-gpu](https://github.com/Bevy-Rust-GPU/bevy-rust-gpu)もあるけどボイラープレート量がどうだろう。

Bevyはバックエンドにwgpuを使っている。
wgpuが実装する規格WebGPUはgeometryやtessellationには対応していない。
MeshShaderが来るかもしれないとか。

Bevyのバックエンドは切り替えられる。もっと生のVulkanとかを触りたくなった時に検討する。
ただしBevyとどこまで連携してるかはまちまちなので注意。
* [bevy_kajiya](https://github.com/seabassjh/bevy-kajiya)で切り替えられる
[kajiya](https://github.com/EmbarkStudios/kajiya)はRustGPUと同じEmbarkStudio製で連携も考えられているが、
kajiya本体もプラグインもexperimentalだしまだまだ開発中っぽい。
* [bevy_vulkano](https://github.com/hakolao/bevy_vulkano)はかなり低レベルから書く必要がありそう。


