import 'package:coolicons/coolicons.dart';
import 'package:flutter/material.dart';

class Profile extends StatelessWidget {
  const Profile({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        child: Row(
      children: [
        Container(
            height: 40,
            width: 40,
            child: CircleAvatar(
              radius: 60.0,
              backgroundColor: Colors.grey,
            )),
        SizedBox(width: 15),
        Text("Max Mustermann"),
        IconButton(onPressed: () {}, icon: Icon(Coolicons.caret_down))
      ],
    ));
  }
}
