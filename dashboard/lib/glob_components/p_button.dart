// ignore_for_file: avoid_unnecessary_containers

import 'package:flutter/material.dart';

class PButton extends StatefulWidget {
  final String text;
  final Color color;
  final Function onTap;
  PButton(this.text, this.color, this.onTap);

  @override
  _PButtonState createState() => _PButtonState();
}

class _PButtonState extends State<PButton> {
  @override
  Widget build(BuildContext context) {
    Color bcolor = widget.color;
    return Container(
        height: 40,
        width: 152,
        decoration: BoxDecoration(
            color: bcolor,
            borderRadius: const BorderRadius.all(Radius.circular(5))),
        child: InkWell(
            onTap: () {
              this.widget.onTap();
            },
            onHover: (value) {
              setState(() {
                if (value) {
                  bcolor = Colors.grey;
                } else {
                  bcolor = widget.color;
                }
              });
            },
            child: Center(
                child: Text(
              widget.text,
              style: const TextStyle(
                  color: Colors.white,
                  fontSize: 16,
                  fontWeight: FontWeight.bold),
            ))));
  }
}
