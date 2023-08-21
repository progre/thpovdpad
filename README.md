# thpovdpad

A tool to control character with cross key (POV hat switch) of game pad for Touhou Project's game. 

東方Projectのゲームにおいて、ゲームパッドの十字キー（POVハットスイッチ）でキャラクターを操作するためのツールです。

アナログスティックの付いたコントローラーなどで十字キーでの操作ができない時に使ってみてください。
東方Project以外のゲームでも使えたり使えなかったりすると思います。
東方地霊殿はDirectInputが動作しない事がありますが、このツールを使うと回避できることがあります。

## Usage

Extract the zip file and place dinput8.dll in the same location as the exe file.

zipファイルを展開してdinput8.dllをexeファイルと同じ場所に配置してください。

## How it works (ja)

DirectInputの処理を乗っ取って、POVハットスイッチの入力をキーボードの入力に変換しています。
