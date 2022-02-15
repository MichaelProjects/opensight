import 'package:dashboard/utils/sizes.dart';
import 'package:flutter/material.dart';

class Explore extends StatefulWidget {
  Explore({Key? key}) : super(key: key);

  @override
  State<Explore> createState() => _ExploreState();
}

class _ExploreState extends State<Explore> {
  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    return SizedBox(
      height: size.height - topbarHeight,
      width: size.width - sidebarWidth,
    );
  }
}
