sabi::sabi! {
絶え間ない 入力: &str = 文字列を含める!("一.txt");

公共 関数 メイン() {
    させて エルフ = 入力.スプリット("\r\n\r\n");
    させて 可変 最大: ベクター<符号なし整数> = デフォルト::デフォルト値();

    のために 単一 で エルフ {
        させて 合計 = 単一.スプリット("\r\n").地図(|項目| 項目.解析する::<符号なし整数>().包みを解く()).和();
        最大.押す(合計);
    }

    最大.可変スライスとして().選別();
    最大.逆行する();

    印刷ライン!("{:?}", &最大[0]);
    印刷ライン!("{:?}", &最大[0..3].イテレータ().和::<符号なし整数>());
}
}