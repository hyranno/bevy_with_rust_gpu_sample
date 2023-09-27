# BevyでRustGPUを使いたい(WIP)

ShaderRefには対応していない

[bevy-rust-gpu](https://github.com/Bevy-Rust-GPU/bevy-rust-gpu)もあるけどボイラープレート量や更新頻度がどうだろう。

Bevyはバックエンドにwgpuを使っている。
wgpuが実装する規格WebGPUはgeometryやtessellationには対応していない。
MeshShaderが来るかもしれないとか。

Bevyのバックエンドは切り替えられる。もっと生のVulkanとかを触りたくなった時に検討する。
ただしBevyとどこまで連携してるかはまちまちなので注意。
* [bevy_kajiya](https://github.com/seabassjh/bevy-kajiya)で切り替えられる
[kajiya](https://github.com/EmbarkStudios/kajiya)はRustGPUと同じEmbarkStudio製で連携も考えられているが、
kajiya本体もプラグインもexperimentalだしまだまだ開発中っぽい。
* [bevy_vulkano](https://github.com/hakolao/bevy_vulkano)はかなり低レベルから書く必要がありそう。


