import React from "react";
import { Modal, Button, Form, Input, Select } from "antd";
import { FiX } from "react-icons/fi";

const { Option } = Select;

const AddRecord = ({ handleCancel, handleOk, records }) => {
  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  return (
    <React.Fragment>
      <Modal
        width={800}
        title={null}
        footer={null}
        closeIcon={<FiX className="close-icon" />}
        visible={records}
        onCancel={handleCancel}
        onOk={handleOk}
      >
        <div className="container-adding-records">
          <Form {...layout} size="large">
            <Form.Item
              label="Sub domain name"
              name="subdomain-name"
              rules={[
                { required: true, message: "Please input sub domain name!" },
              ]}
            >
              <Input placeholder="text here ..." size="large" />
            </Form.Item>
            <Form.Item
              label="Adress"
              name="address"
              rules={[{ required: true, message: "Please input the address!" }]}
            >
              <Input placeholder="text here ..." size="large" />
            </Form.Item>
            <Form.Item
              label="Type"
              name="type"
              rules={[
                {
                  required: true,
                  message: "Please select your type!",
                },
              ]}
            >
              <Select defaultValue="A" size="large" className="select-option">
                <Option value="A">A</Option>
                <Option value="MX 10">MX 10</Option>
                <Option value="CNAME">CNAME</Option>
              </Select>
            </Form.Item>
            <Form.Item>
              <Button
                className="button-apply"
                size="large"
                htmlType="button"
                type="primary"
              >
                Submit
              </Button>
            </Form.Item>
          </Form>
        </div>
      </Modal>
    </React.Fragment>
  );
};

export default AddRecord;
