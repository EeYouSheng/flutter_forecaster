enum PredictionType {
  weekly,
  monthly;

  String get name => switch (this) {
        PredictionType.weekly => 'Weekly',
        PredictionType.monthly => 'Monthly',
      };
}