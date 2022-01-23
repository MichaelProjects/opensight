import 'package:dashboard/screens/app_select/components/appselector.dart';
import 'package:dashboard/screens/app_select/components/decoration_spacer.dart';
import 'package:dashboard/screens/overlay/topbar.dart';
import 'package:flutter/material.dart';

class AppSelect extends StatefulWidget {
  AppSelect({Key? key}) : super(key: key);

  @override
  _AppSelectState createState() => _AppSelectState();
}

class _AppSelectState extends State<AppSelect> {
  @override
  Widget build(BuildContext context) {
    var data = MediaQuery.of(context).size;
    return Scaffold(
        body: Container(
            child: Column(
      children: [
        const TopBar(),
        Stack(
          clipBehavior: Clip.none,
          children: [
            DecorationSpacer(),
            Positioned(
                top: 50, left: (data.width / 100) * 43, child: AppSelector())
          ],
        )
      ],
    )));
  }
}
