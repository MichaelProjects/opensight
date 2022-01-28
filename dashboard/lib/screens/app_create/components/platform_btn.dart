import 'package:flutter/material.dart';

class PlatformBtn extends StatelessWidget {
  final String asset;
  final Color color;
  Function onSelect;
  PlatformBtn(this.asset, this.color, this.onSelect, {Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        margin: EdgeInsets.all(10),
        height: 40,
        width: 40,
        decoration: BoxDecoration(borderRadius: BorderRadius.circular(40)),
        child: InkWell(
            onHover: (value) {
              print(value);
            },
            onTap: () => onSelect(),
            child: Column(
              children: [Image.asset(asset, height: 40, width: 40)],
            )));
  }
}
