import 'package:flutter/material.dart';

class SidebarContainer extends StatelessWidget {
  final Widget child;
  const SidebarContainer({Key? key, required this.child}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        margin: const EdgeInsets.all(10),
        padding: const EdgeInsets.all(2),
        width: 210,
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(10),
          color: Theme.of(context).primaryColor,
        ),
        child: child);
  }
}
