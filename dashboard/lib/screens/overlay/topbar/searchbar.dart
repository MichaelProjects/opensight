import 'package:coolicons/coolicons.dart';
import 'package:flutter/material.dart';

class Searchbar extends StatefulWidget {
  const Searchbar({Key? key}) : super(key: key);

  @override
  _SearchbarState createState() => _SearchbarState();
}

class _SearchbarState extends State<Searchbar> {
  @override
  Widget build(BuildContext context) {
    var textField = const SizedBox(
        height: 50,
        width: 250,
        child: TextField(
          decoration:
              InputDecoration(hintText: "Search...", border: InputBorder.none),
        ));

    return Container(
        width: 300,
        height: 60,
        margin: const EdgeInsets.all(10),
        child: Row(children: [const Icon(Coolicons.search), textField]));
  }
}
