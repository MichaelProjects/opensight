import 'package:flutter/material.dart';

class CancleButton extends StatefulWidget {
  @override
  State<CancleButton> createState() => CancleButtonState();
}

class CancleButtonState extends State<CancleButton> {
  @override
  Widget build(BuildContext context) {
    return Container(
        margin: EdgeInsets.all(10),
        child: InkWell(
            onHover: (value) {},
            onTap: () {
              Navigator.pop(context);
            },
            child:
                Text("Cancle", style: Theme.of(context).textTheme.subtitle1)));
  }
}
