import 'package:dashboard/screens/overlay/topbar/logo_container.dart';
import 'package:dashboard/screens/overlay/topbar/profile.dart';
import 'package:dashboard/screens/overlay/topbar/searchbar.dart';
import 'package:dashboard/screens/overlay/topbar/timeline.dart';
import 'package:flutter/material.dart';

class TopBar extends StatefulWidget {
  const TopBar({Key? key}) : super(key: key);

  @override
  _TopBarState createState() => _TopBarState();
}

class _TopBarState extends State<TopBar> {
  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    return Container(
        decoration: BoxDecoration(
            borderRadius: const BorderRadius.all(Radius.circular(5)),
            color: Theme.of(context).primaryColor),
        width: size.width,
        height: 70,
        child: Row(
          children: [
            const LogoContainer(),
            const Timeline(),
            const Searchbar(),
            Expanded(child: Container()),
            const Profile()
          ],
        ));
  }
}
