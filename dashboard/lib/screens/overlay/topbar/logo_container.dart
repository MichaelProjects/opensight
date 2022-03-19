import 'package:flutter/material.dart';
import 'package:routemaster/routemaster.dart';

class LogoContainer extends StatelessWidget {
  const LogoContainer({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
        onTap: () {
          Routemaster.of(context).push('/');
        },
        child: Container(
            margin: const EdgeInsets.only(left: 20),
            width: 175,
            height: 50,
            child: Image.asset("img/opensight.png")));
  }
}
