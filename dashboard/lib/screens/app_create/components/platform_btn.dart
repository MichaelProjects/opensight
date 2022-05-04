import 'package:flutter/material.dart';

class PlatformBtn extends StatefulWidget {
  final String asset;
  final Color color;
  Function onSelect;
  PlatformBtn(this.asset, this.color, this.onSelect, {Key? key})
      : super(key: key);

  @override
  State<PlatformBtn> createState() => _PlatformBtnState();
}

class _PlatformBtnState extends State<PlatformBtn> {
  @override
  Widget build(BuildContext context) {
    return Container(
        margin: const EdgeInsets.all(5),
        padding: const EdgeInsets.all(5),
        height: 50,
        width: 50,
        decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(5), color: widget.color),
        child: InkWell(
            onHover: (value) {
              print(value);
            },
            onTap: () => widget.onSelect(),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [Image.asset(widget.asset, height: 40, width: 40)],
            )));
  }
}
