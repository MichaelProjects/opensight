// ignore_for_file: prefer_const_constructors

import 'package:flutter/material.dart';

class DecorationSpacer extends StatelessWidget {
  const DecorationSpacer({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var data = MediaQuery.of(context).size;
    return Container(
        decoration: BoxDecoration(
          color: Theme.of(context).primaryColor,
          borderRadius: BorderRadius.only(
            bottomLeft: Radius.circular(20),
            bottomRight: Radius.circular(20),
          ),
        ),
        width: data.width,
        height: 210,
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Image.asset("img/opensight_appselector.png"),
            SizedBox(width: data.width * 0.3),
            Image.asset("img/opensight_appselector2.png")
          ],
        ));
  }
}
