import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

class CustomBarChart extends StatefulWidget {
  @override
  State<CustomBarChart> createState() => _BarChartState();
}

class _BarChartState extends State<CustomBarChart> {
  @override
  Widget build(BuildContext context) {
    BarChartData mainData() {
      return BarChartData(
        barGroups: [
          BarChartGroupData(x: 1, barRods: [
            BarChartRodData(y: 100),
          ])
        ],
        gridData: FlGridData(show: false),
        borderData: FlBorderData(show: false),
      );
    }

    return BarChart(mainData());
  }
}
