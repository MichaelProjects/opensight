import 'package:coolicons/coolicons.dart';
import 'package:flutter/material.dart';

class Searchbar extends StatefulWidget {
  Searchbar({Key? key}) : super(key: key);

  @override
  _SearchbarState createState() => _SearchbarState();
}

class _SearchbarState extends State<Searchbar> {
  @override
  Widget build(BuildContext context) {
    var textField = Container(
        height: 50,
        width: 250,
        child: TextField(
          decoration:
              InputDecoration(hintText: "Search...", border: InputBorder.none),
        ));

    return Container(
        width: 300,
        height: 60,
        margin: EdgeInsets.all(10),
        child: Row(children: [Icon(Coolicons.search), textField]));
  }
}
