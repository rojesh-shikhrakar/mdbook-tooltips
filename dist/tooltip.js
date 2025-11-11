const tooltip = document.getElementById('tooltipText');
  const target = document.getElementById('tooltipTarget');

  target.addEventListener('mouseenter', () => {
    tooltip.style.visibility = 'visible';
    tooltip.style.opacity = '1';
  });

  target.addEventListener('mouseleave', () => {
    tooltip.style.visibility = 'hidden';
    tooltip.style.opacity = '0';
  });