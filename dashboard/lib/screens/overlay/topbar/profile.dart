import 'package:flutter/material.dart';

class Profile extends StatelessWidget {
  const Profile({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        child: Row(
      children: [
        Container(
            height: 54,
            width: 54,
            child: CircleAvatar(
              radius: 60.0,
              backgroundColor: Colors.grey,
            )),
        SizedBox(width: 15),
        Text("Max Mustermann")
      ],
    ));
  }
}
