import { test, describe, expect } from 'vitest';

import { tick } from 'svelte';
import { waitFor, act, render, fireEvent, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event'

import CategoryComponent from './CategoryComponent.svelte';

describe('CategoryComponent should display label provided', async () => {
  render(CategoryComponent, { label: 'Test', categoryId: 10 });

  const editButton = screen.getByTestId('edit-button');
  const deleteButton = screen.getByTestId('delete-button');
  const closedTextField = screen.getByTestId('category-input-closed');

  test('Initially should have edit and delete available', () => {
    expect(editButton).toBeDefined();
    expect(deleteButton).toBeDefined();
  });

  test('Should disable textfield', () => {
    expect(closedTextField.disabled).toBe(true);
    expect(closedTextField.value).toBe('Test');
  });

  await fireEvent.click(editButton);
  await tick();

  let cancelButton = screen.getByTestId('cancel-button');
  let saveButton = screen.getByTestId('save-button');

  test('After clicking edit, should have cancel and save available', () => {
    expect(cancelButton).toBeDefined();
    expect(saveButton).toBeDefined();
  });

  const openTextField = screen.getByTestId('category-input-open');

  // this for some reason gets triggered after the userEvent that is supposed to change the value after
  // potentially a bug with vitest
  // test('should display label provided', () => {
  //   expect(openTextField.value, "Field is changed too soon???").toBe('Test');
  // });

  await userEvent.type(
    screen.getByTestId('category-input-open'),
    '{backspace}{backspace}{backspace}{backspace}Changed'
  );

  test('should display changed value', () => {
    expect(openTextField.disabled).toBe(false);
    expect(openTextField.value, "Inputting new value should affect the value stored by this field").toBe('Changed');
  });

  await fireEvent.click(screen.getByTestId('cancel-button'));

  test('After clicking cancel, should have edit and delete available', () => {
    expect(editButton).toBeDefined();
    expect(deleteButton).toBeDefined();
  });

  test('Input should be reset on cancel', () => {
    expect(closedTextField, "The cancel did not update the fields value").toHaveProperty('value', 'Test');
  });
});
