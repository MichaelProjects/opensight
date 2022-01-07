import 'package:flutter/material.dart';

class ChartBox extends StatelessWidget {
  final String chartName;
  final Widget child;
  const ChartBox({Key? key, required this.chartName, required this.child})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        padding: const EdgeInsets.all(10),
        decoration: BoxDecoration(
            color: Theme.of(context).backgroundColor,
            borderRadius: const BorderRadius.all(Radius.circular(15))),
        child: Column(
          children: [SelectableText(chartName), child],
        ));
  }
}
