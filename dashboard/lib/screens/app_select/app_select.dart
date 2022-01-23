import 'package:dashboard/controllers/dashboard/sidebar/app_controller.dart';
import 'package:dashboard/screens/app_select/components/appselector.dart';
import 'package:dashboard/screens/app_select/components/decoration_spacer.dart';
import 'package:dashboard/screens/overlay/topbar.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class AppSelect extends StatefulWidget {
  AppSelect({Key? key}) : super(key: key);

  @override
  _AppSelectState createState() => _AppSelectState();
}

class _AppSelectState extends State<AppSelect> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: Container(
            child: Column(
      children: [const TopBar(), DecorationSpacer(), AppSelector()],
    )));
  }
}