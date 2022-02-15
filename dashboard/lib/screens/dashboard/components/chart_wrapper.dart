import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

class ChartWrapper extends StatelessWidget {
  Widget child;
  List<FlSpot> data;
  String title;
  bool loading;
  ChartWrapper(
      {required this.child,
      required this.title,
      required this.data,
      required this.loading});

  @override
  Widget build(BuildContext context) {
    return Container(
        color: Theme.of(context).primaryColor,
        child: Column(children: [
          Container(
            child: Center(child: Text(title)),
            padding:
                const EdgeInsets.only(left: 8, right: 8, top: 20, bottom: 8),
          ),
          Builder(
            builder: (BuildContext context) {
              if (data.isEmpty) {
                return Container(
                  child: const Center(child: CircularProgressIndicator()),
                  padding: const EdgeInsets.only(
                      left: 8, right: 8, top: 8, bottom: 8),
                );
              } else {
                return Expanded(
                    child: Padding(
                        padding: const EdgeInsets.only(
                            right: 18, top: 18, bottom: 18),
                        child: child));
              }
            },
          ),
        ]));
  }
}
