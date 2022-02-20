import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

class CustomPieChart extends StatefulWidget {
  final List<PieChartSectionData> sections;
  CustomPieChart(this.sections);

  @override
  State<CustomPieChart> createState() => _CustomPieChartState();
}

class _CustomPieChartState extends State<CustomPieChart> {
  @override
  Widget build(BuildContext context) {
    PieChartData mainData() {
      return PieChartData(sections: widget.sections);
    }

    return PieChart(
      mainData(),
      swapAnimationDuration: const Duration(milliseconds: 150), // Optional
      swapAnimationCurve: Curves.linear, // Optional
    );
  }
}
